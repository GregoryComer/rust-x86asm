use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn maxss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 228], OperandSize::Dword)
}

fn maxss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ECX, 1209226046, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 153, 62, 83, 19, 72], OperandSize::Dword)
}

fn maxss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 227], OperandSize::Qword)
}

fn maxss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 95, 28, 247], OperandSize::Qword)
}


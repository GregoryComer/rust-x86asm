use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 237], OperandSize::Dword)
}

fn pminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 20, 211], OperandSize::Dword)
}

fn pminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 227], OperandSize::Qword)
}

fn pminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 58, 20, 246], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 237], OperandSize::Dword)
}

fn pminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1075854401, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 36, 85, 65, 60, 32, 64], OperandSize::Dword)
}

fn pminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 218], OperandSize::Qword)
}

fn pminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 258114525, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 56, 12, 197, 221, 131, 98, 15], OperandSize::Qword)
}


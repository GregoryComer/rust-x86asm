use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movsldup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 239], OperandSize::Dword)
}

fn movsldup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 36, 136], OperandSize::Dword)
}

fn movsldup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 244], OperandSize::Qword)
}

fn movsldup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSLDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 738533478, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 18, 44, 69, 102, 32, 5, 44], OperandSize::Qword)
}


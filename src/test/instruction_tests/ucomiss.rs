use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 225], OperandSize::Dword)
}

fn ucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 43], OperandSize::Dword)
}

fn ucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 205], OperandSize::Qword)
}

fn ucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 30396721, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 155, 49, 209, 207, 1], OperandSize::Qword)
}


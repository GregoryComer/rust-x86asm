use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 211, 27], OperandSize::Dword)
}

#[test]
fn pinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Byte), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 60, 145, 116], OperandSize::Dword)
}

#[test]
fn pinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 253, 126], OperandSize::Qword)
}

#[test]
fn pinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 32, 3, 51], OperandSize::Qword)
}


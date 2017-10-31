use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectDisplaced(EAX, 881705390, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 176, 174, 193, 141, 52], OperandSize::Dword)
}

#[test]
fn clwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 52, 131], OperandSize::Qword)
}


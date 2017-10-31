use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1196519679, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 156, 67, 255, 112, 81, 71], OperandSize::Dword)
}

#[test]
fn bndmk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND3)), operand2: Some(IndirectDisplaced(RBX, 609631360, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 155, 128, 60, 86, 36], OperandSize::Qword)
}


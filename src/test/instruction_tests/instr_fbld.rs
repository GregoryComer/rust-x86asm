use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 22445, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 161, 173, 87], OperandSize::Word)
}

#[test]
fn fbld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 36, 150], OperandSize::Dword)
}

#[test]
fn fbld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectDisplaced(RDI, 2058387172, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 167, 228, 126, 176, 122], OperandSize::Qword)
}


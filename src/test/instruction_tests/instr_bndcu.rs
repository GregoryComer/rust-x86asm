use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND0)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 199], OperandSize::Dword)
}

#[test]
fn bndcu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 4, 145], OperandSize::Dword)
}

#[test]
fn bndcu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND0)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 197], OperandSize::Qword)
}

#[test]
fn bndcu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND0)), operand2: Some(IndirectDisplaced(RBX, 2083881688, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 131, 216, 130, 53, 124], OperandSize::Qword)
}


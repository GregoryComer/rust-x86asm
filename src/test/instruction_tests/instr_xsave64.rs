use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsave64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE64, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 478519929, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 174, 36, 197, 121, 162, 133, 28], OperandSize::Qword)
}


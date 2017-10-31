use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxsave64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE64, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 84854522, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 174, 132, 71, 250, 198, 14, 5], OperandSize::Qword)
}


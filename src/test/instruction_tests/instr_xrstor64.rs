use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstor64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR64, operand1: Some(IndirectDisplaced(RAX, 1476095583, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 174, 168, 95, 110, 251, 87], OperandSize::Qword)
}


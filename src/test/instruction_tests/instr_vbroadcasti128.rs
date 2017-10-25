use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vbroadcasti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 824242822, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 4, 213, 134, 242, 32, 49], OperandSize::Dword)
}

#[test]
fn vbroadcasti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBROADCASTI128, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1400713463, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 90, 36, 253, 247, 48, 125, 83], OperandSize::Qword)
}


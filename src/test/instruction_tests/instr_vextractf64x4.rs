use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 27, 250, 77], OperandSize::Dword)
}

#[test]
fn vextractf64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 11, 80], OperandSize::Dword)
}

#[test]
fn vextractf64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM23)), operand2: Some(Direct(ZMM11)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 51, 253, 206, 27, 223, 52], OperandSize::Qword)
}

#[test]
fn vextractf64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectDisplaced(RAX, 1609800228, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM10)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 115, 253, 72, 27, 144, 36, 154, 243, 95, 118], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 27, 243, 33], OperandSize::Dword)
}

#[test]
fn vextractf64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectDisplaced(EDI, 1633666429, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 135, 125, 197, 95, 97, 125], OperandSize::Dword)
}

#[test]
fn vextractf64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(Direct(YMM26)), operand2: Some(Direct(ZMM9)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 253, 204, 27, 202, 43], OperandSize::Qword)
}

#[test]
fn vextractf64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF64x4, operand1: Some(IndirectDisplaced(RDX, 663132638, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 27, 178, 222, 153, 134, 39, 62], OperandSize::Qword)
}


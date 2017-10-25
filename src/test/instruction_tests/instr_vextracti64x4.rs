use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti64x4_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 59, 203, 21], OperandSize::Dword)
}

#[test]
fn vextracti64x4_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledDisplaced(ESI, Two, 571941532, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 253, 72, 59, 28, 117, 156, 34, 23, 34, 4], OperandSize::Dword)
}

#[test]
fn vextracti64x4_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(Direct(YMM16)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 253, 206, 59, 200, 121], OperandSize::Qword)
}

#[test]
fn vextracti64x4_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI64x4, operand1: Some(IndirectScaledDisplaced(RBX, Two, 646671603, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 253, 72, 59, 52, 93, 243, 108, 139, 38, 38], OperandSize::Qword)
}


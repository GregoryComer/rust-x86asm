use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 206, 35, 229, 74], OperandSize::Dword)
}

#[test]
fn vshuff64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 2076644997, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 204, 35, 184, 133, 22, 199, 123, 10], OperandSize::Dword)
}

#[test]
fn vshuff64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 1605136080, Some(OperandSize::Qword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 223, 35, 139, 208, 110, 172, 95, 125], OperandSize::Dword)
}

#[test]
fn vshuff64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 173, 193, 35, 214, 62], OperandSize::Qword)
}

#[test]
fn vshuff64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 189, 196, 35, 24, 47], OperandSize::Qword)
}

#[test]
fn vshuff64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 213, 221, 35, 36, 153, 109], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshuff64x2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 202, 35, 225, 6], OperandSize::Dword)
}

#[test]
fn vshuff64x2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 1268228801, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 35, 129, 193, 162, 151, 75, 97], OperandSize::Dword)
}

#[test]
fn vshuff64x2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 222, 35, 31, 96], OperandSize::Dword)
}

#[test]
fn vshuff64x2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM26)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 133, 195, 35, 234, 87], OperandSize::Qword)
}

#[test]
fn vshuff64x2_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 527843639, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 133, 203, 35, 20, 141, 55, 65, 118, 31, 30], OperandSize::Qword)
}

#[test]
fn vshuff64x2_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFF64x2, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 189, 218, 35, 4, 187, 111], OperandSize::Qword)
}


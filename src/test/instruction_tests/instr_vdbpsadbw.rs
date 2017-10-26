use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdbpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 138, 66, 243, 116], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1639252133, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 138, 66, 188, 127, 165, 0, 181, 97, 102], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM21)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 35, 85, 139, 66, 253, 97], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 93, 143, 66, 39, 53], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 175, 66, 225, 91], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 117, 171, 66, 12, 206, 101], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM29)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 131, 53, 173, 66, 245, 36], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 77, 175, 66, 12, 135, 53], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 203, 66, 199, 59], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 201, 66, 49, 124], OperandSize::Dword)
}

#[test]
fn vdbpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM21)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 85, 197, 66, 229, 93], OperandSize::Qword)
}

#[test]
fn vdbpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDBPSADBW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 198, 66, 51, 70], OperandSize::Qword)
}


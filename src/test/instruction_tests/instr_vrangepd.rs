use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 143, 80, 232, 119], OperandSize::Dword)
}

#[test]
fn vrangepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1984469156, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 142, 80, 12, 157, 164, 152, 72, 118, 114], OperandSize::Dword)
}

#[test]
fn vrangepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 971276867, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 159, 80, 12, 85, 67, 130, 228, 57, 91], OperandSize::Dword)
}

#[test]
fn vrangepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 35, 205, 131, 80, 254, 40], OperandSize::Qword)
}

#[test]
fn vrangepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 101388293, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 138, 80, 156, 79, 5, 16, 11, 6, 77], OperandSize::Qword)
}

#[test]
fn vrangepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 189, 148, 80, 41, 91], OperandSize::Qword)
}

#[test]
fn vrangepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 175, 80, 235, 70], OperandSize::Dword)
}

#[test]
fn vrangepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 446567546, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 175, 80, 172, 191, 122, 20, 158, 26, 117], OperandSize::Dword)
}

#[test]
fn vrangepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1207759690, Some(OperandSize::Qword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 197, 191, 80, 12, 77, 74, 243, 252, 71, 77], OperandSize::Dword)
}

#[test]
fn vrangepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM24)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 19, 197, 172, 80, 192, 9], OperandSize::Qword)
}

#[test]
fn vrangepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDI, 1181246365, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 141, 167, 80, 143, 157, 99, 104, 70, 66], OperandSize::Qword)
}

#[test]
fn vrangepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 173, 177, 80, 44, 152, 51], OperandSize::Qword)
}

#[test]
fn vrangepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 158, 80, 204, 65], OperandSize::Dword)
}

#[test]
fn vrangepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 726957866, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 203, 80, 12, 93, 42, 127, 84, 43, 47], OperandSize::Dword)
}

#[test]
fn vrangepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 39182400, Some(OperandSize::Qword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 221, 222, 80, 44, 181, 64, 224, 85, 2, 59], OperandSize::Dword)
}

#[test]
fn vrangepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 131, 213, 150, 80, 223, 15], OperandSize::Qword)
}

#[test]
fn vrangepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 99, 221, 193, 80, 22, 5], OperandSize::Qword)
}

#[test]
fn vrangepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 715187144, Some(OperandSize::Qword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 173, 223, 80, 28, 77, 200, 227, 160, 42, 41], OperandSize::Qword)
}


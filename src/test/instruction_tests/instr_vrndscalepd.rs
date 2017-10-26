use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 137, 9, 199, 20], OperandSize::Dword)
}

#[test]
fn vrndscalepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1648048349, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 9, 60, 133, 221, 56, 59, 98, 2], OperandSize::Dword)
}

#[test]
fn vrndscalepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1568280153, Some(OperandSize::Qword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 154, 9, 4, 189, 89, 14, 122, 93, 71], OperandSize::Dword)
}

#[test]
fn vrndscalepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 19, 253, 141, 9, 202, 110], OperandSize::Qword)
}

#[test]
fn vrndscalepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 666410346, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 142, 9, 12, 85, 106, 157, 184, 39, 68], OperandSize::Qword)
}

#[test]
fn vrndscalepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RCX, 809718236, Some(OperandSize::Qword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 159, 9, 161, 220, 81, 67, 48, 54], OperandSize::Qword)
}

#[test]
fn vrndscalepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 173, 9, 241, 117], OperandSize::Dword)
}

#[test]
fn vrndscalepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 9, 51, 68], OperandSize::Dword)
}

#[test]
fn vrndscalepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2034814387, Some(OperandSize::Qword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 9, 52, 181, 179, 205, 72, 121, 6], OperandSize::Dword)
}

#[test]
fn vrndscalepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 51, 253, 173, 9, 202, 46], OperandSize::Qword)
}

#[test]
fn vrndscalepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 140840963, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 253, 172, 9, 164, 66, 3, 16, 101, 8, 76], OperandSize::Qword)
}

#[test]
fn vrndscalepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1031385925, Some(OperandSize::Qword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 187, 9, 180, 254, 69, 179, 121, 61, 9], OperandSize::Qword)
}

#[test]
fn vrndscalepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 157, 9, 221, 74], OperandSize::Dword)
}

#[test]
fn vrndscalepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 9, 4, 249, 85], OperandSize::Dword)
}

#[test]
fn vrndscalepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 2125649193, Some(OperandSize::Qword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 9, 44, 117, 41, 213, 178, 126, 55], OperandSize::Dword)
}

#[test]
fn vrndscalepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM22)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 253, 154, 9, 222, 119], OperandSize::Qword)
}

#[test]
fn vrndscalepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RSI, 34897185, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 253, 204, 9, 182, 33, 125, 20, 2, 112], OperandSize::Qword)
}

#[test]
fn vrndscalepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALEPD, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 219, 9, 19, 111], OperandSize::Qword)
}


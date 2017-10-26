use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrangepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 229, 139, 80, 196, 116], OperandSize::Dword)
}

#[test]
fn vrangepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 2127174378, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 141, 80, 20, 133, 234, 26, 202, 126, 110], OperandSize::Dword)
}

#[test]
fn vrangepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1820881283, Some(OperandSize::Qword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 229, 156, 80, 164, 152, 131, 113, 136, 108, 99], OperandSize::Dword)
}

#[test]
fn vrangepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM26)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 245, 143, 80, 226, 78], OperandSize::Qword)
}

#[test]
fn vrangepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1175432459, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 189, 139, 80, 132, 214, 11, 173, 15, 70, 0], OperandSize::Qword)
}

#[test]
fn vrangepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 151, 80, 15, 46], OperandSize::Qword)
}

#[test]
fn vrangepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 175, 80, 229, 62], OperandSize::Dword)
}

#[test]
fn vrangepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDI, 661084146, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 197, 172, 80, 183, 242, 87, 103, 39, 86], OperandSize::Dword)
}

#[test]
fn vrangepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 740001698, Some(OperandSize::Qword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 205, 190, 80, 12, 117, 162, 135, 27, 44, 87], OperandSize::Dword)
}

#[test]
fn vrangepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM11)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 83, 141, 166, 80, 211, 47], OperandSize::Qword)
}

#[test]
fn vrangepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1593214511, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 149, 174, 80, 156, 118, 47, 134, 246, 94, 91], OperandSize::Qword)
}

#[test]
fn vrangepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RCX, 1802674152, Some(OperandSize::Qword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 205, 190, 80, 137, 232, 159, 114, 107, 41], OperandSize::Qword)
}

#[test]
fn vrangepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 156, 80, 217, 113], OperandSize::Dword)
}

#[test]
fn vrangepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1290652005, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 205, 207, 80, 52, 181, 101, 201, 237, 76, 3], OperandSize::Dword)
}

#[test]
fn vrangepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1295331607, Some(OperandSize::Qword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 217, 80, 188, 122, 23, 49, 53, 77, 47], OperandSize::Dword)
}

#[test]
fn vrangepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM18)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 51, 173, 159, 80, 234, 112], OperandSize::Qword)
}

#[test]
fn vrangepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RDX, 331402921, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 245, 205, 80, 170, 169, 206, 192, 19, 37], OperandSize::Qword)
}

#[test]
fn vrangepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 955098185, Some(OperandSize::Qword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 189, 210, 80, 140, 78, 73, 164, 237, 56, 40], OperandSize::Qword)
}


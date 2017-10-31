use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 70, 207], OperandSize::Dword)
}

#[test]
fn vpsravd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 2109254241, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 70, 154, 97, 170, 184, 125], OperandSize::Dword)
}

#[test]
fn vpsravd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 70, 197], OperandSize::Qword)
}

#[test]
fn vpsravd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1486530179, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 70, 20, 197, 131, 166, 154, 88], OperandSize::Qword)
}

#[test]
fn vpsravd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 70, 202], OperandSize::Dword)
}

#[test]
fn vpsravd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 548954638, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 70, 162, 14, 98, 184, 32], OperandSize::Dword)
}

#[test]
fn vpsravd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 70, 215], OperandSize::Qword)
}

#[test]
fn vpsravd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 2126875022, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 70, 131, 142, 137, 197, 126], OperandSize::Qword)
}

#[test]
fn vpsravd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 70, 251], OperandSize::Dword)
}

#[test]
fn vpsravd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1040304352, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 70, 4, 205, 224, 200, 1, 62], OperandSize::Dword)
}

#[test]
fn vpsravd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 159, 70, 23], OperandSize::Dword)
}

#[test]
fn vpsravd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 77, 135, 70, 208], OperandSize::Qword)
}

#[test]
fn vpsravd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 109, 130, 70, 41], OperandSize::Qword)
}

#[test]
fn vpsravd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RCX, 1829735012, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 155, 70, 185, 100, 138, 15, 109], OperandSize::Qword)
}

#[test]
fn vpsravd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 70, 214], OperandSize::Dword)
}

#[test]
fn vpsravd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 70, 60, 186], OperandSize::Dword)
}

#[test]
fn vpsravd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1718430006, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 70, 164, 87, 54, 41, 109, 102], OperandSize::Dword)
}

#[test]
fn vpsravd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 45, 167, 70, 204], OperandSize::Qword)
}

#[test]
fn vpsravd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 218532672, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 85, 173, 70, 60, 117, 64, 139, 6, 13], OperandSize::Qword)
}

#[test]
fn vpsravd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 906050379, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 70, 52, 93, 75, 59, 1, 54], OperandSize::Qword)
}

#[test]
fn vpsravd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 70, 219], OperandSize::Dword)
}

#[test]
fn vpsravd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 537917378, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 70, 135, 194, 247, 15, 32], OperandSize::Dword)
}

#[test]
fn vpsravd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 223, 70, 20, 214], OperandSize::Dword)
}

#[test]
fn vpsravd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 117, 196, 70, 211], OperandSize::Qword)
}

#[test]
fn vpsravd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 155594974, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 69, 199, 70, 12, 181, 222, 48, 70, 9], OperandSize::Qword)
}

#[test]
fn vpsravd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 5, 215, 70, 52, 112], OperandSize::Qword)
}


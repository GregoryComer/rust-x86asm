use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 70, 210], OperandSize::Dword)
}

#[test]
fn vpsravd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 2137037011, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 70, 130, 211, 152, 96, 127], OperandSize::Dword)
}

#[test]
fn vpsravd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 70, 200], OperandSize::Qword)
}

#[test]
fn vpsravd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 70, 25], OperandSize::Qword)
}

#[test]
fn vpsravd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 70, 194], OperandSize::Dword)
}

#[test]
fn vpsravd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 70, 26], OperandSize::Dword)
}

#[test]
fn vpsravd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 70, 200], OperandSize::Qword)
}

#[test]
fn vpsravd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RAX, 1176251563, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 70, 184, 171, 44, 28, 70], OperandSize::Qword)
}

#[test]
fn vpsravd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 70, 221], OperandSize::Dword)
}

#[test]
fn vpsravd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 854542088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 70, 178, 8, 71, 239, 50], OperandSize::Dword)
}

#[test]
fn vpsravd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 155, 70, 22], OperandSize::Dword)
}

#[test]
fn vpsravd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 109, 140, 70, 234], OperandSize::Qword)
}

#[test]
fn vpsravd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 125944468, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 13, 132, 70, 148, 66, 148, 194, 129, 7], OperandSize::Qword)
}

#[test]
fn vpsravd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 101, 154, 70, 32], OperandSize::Qword)
}

#[test]
fn vpsravd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 169, 70, 224], OperandSize::Dword)
}

#[test]
fn vpsravd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1111370192, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 171, 70, 12, 189, 208, 41, 62, 66], OperandSize::Dword)
}

#[test]
fn vpsravd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 426828315, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 186, 70, 168, 27, 226, 112, 25], OperandSize::Dword)
}

#[test]
fn vpsravd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 61, 170, 70, 214], OperandSize::Qword)
}

#[test]
fn vpsravd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 72579763, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 162, 70, 140, 118, 179, 122, 83, 4], OperandSize::Qword)
}

#[test]
fn vpsravd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDI, 903502163, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 117, 180, 70, 167, 83, 89, 218, 53], OperandSize::Qword)
}

#[test]
fn vpsravd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 70, 246], OperandSize::Dword)
}

#[test]
fn vpsravd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1577535939, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 70, 60, 77, 195, 73, 7, 94], OperandSize::Dword)
}

#[test]
fn vpsravd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 437503131, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 219, 70, 52, 125, 155, 196, 19, 26], OperandSize::Dword)
}

#[test]
fn vpsravd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 69, 198, 70, 251], OperandSize::Qword)
}

#[test]
fn vpsravd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1737793862, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 29, 204, 70, 188, 187, 70, 161, 148, 103], OperandSize::Qword)
}

#[test]
fn vpsravd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1183485095, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 61, 217, 70, 180, 187, 167, 140, 138, 70], OperandSize::Qword)
}


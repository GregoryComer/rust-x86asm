use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 167, 226], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 167, 59], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 167, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 834731833, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 167, 164, 241, 57, 255, 192, 49], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 167, 247], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 167, 60, 144], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 167, 250], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1346822898, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 167, 4, 141, 242, 226, 70, 80], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 167, 216], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1135909187, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 167, 145, 67, 153, 180, 67], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1986886148, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 159, 167, 140, 64, 4, 122, 109, 118], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 245, 142, 167, 232], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectDisplaced(RDI, 861182949, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 189, 135, 167, 135, 229, 155, 84, 51], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1584601645, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 197, 154, 167, 44, 197, 45, 26, 115, 94], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 167, 209], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 171, 167, 11], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 185, 167, 44, 155], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 229, 175, 167, 205], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 149, 166, 167, 28, 137], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1264360410, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 188, 167, 140, 198, 218, 155, 92, 75], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 253, 167, 195], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDI, 825109886, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 167, 135, 126, 45, 46, 49], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 909265070, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 223, 167, 4, 77, 174, 72, 50, 54], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 157, 147, 167, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 157, 199, 167, 35], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1229256180, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 205, 211, 167, 140, 131, 244, 245, 68, 73], OperandSize::Qword)
}


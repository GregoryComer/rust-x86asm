use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 151, 246], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 1717947657, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 151, 134, 9, 205, 101, 102], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 151, 243], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDI, 778917272, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 151, 183, 152, 85, 109, 46], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 151, 247], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 151, 62], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 151, 240], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 2085153660, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 151, 142, 124, 235, 72, 124], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 151, 192], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDX, 360329993, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 151, 130, 9, 51, 122, 21], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1129081031, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 154, 151, 172, 208, 199, 104, 76, 67], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 229, 134, 151, 205], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 157, 139, 151, 35], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 149, 151, 9], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 151, 210], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 1993669873, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 151, 160, 241, 252, 212, 118], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 278486901, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 189, 151, 160, 117, 95, 153, 16], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 237, 170, 151, 220], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2110209500, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 161, 151, 60, 141, 220, 61, 199, 125], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 38267090, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 197, 181, 151, 156, 240, 210, 232, 71, 2], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 185, 151, 216], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 306831084, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 151, 156, 82, 236, 222, 73, 18], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1082571092, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 222, 151, 36, 133, 84, 185, 134, 64], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 149, 223, 151, 195], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RDI, 387544748, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 194, 151, 175, 172, 118, 25, 23], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 173, 223, 151, 23], OperandSize::Qword)
}


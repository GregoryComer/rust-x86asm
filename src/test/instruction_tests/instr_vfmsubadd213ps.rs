use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 167, 226], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2036567329, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 167, 52, 157, 33, 141, 99, 121], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 167, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 167, 36, 91], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 216], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 74138595, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 167, 182, 227, 67, 107, 4], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 246], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 167, 52, 122], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 167, 200], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1588814718, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 167, 60, 205, 126, 99, 179, 94], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 153, 167, 51], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 117, 138, 167, 246], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 135, 167, 47], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 101, 146, 167, 20, 127], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 167, 219], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1927943458, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 167, 28, 253, 34, 21, 234, 114], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 187, 167, 22], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 101, 161, 167, 224], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RSI, 131801299, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 21, 174, 167, 142, 211, 32, 219, 7], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1504801353, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 45, 182, 167, 20, 221, 73, 114, 177, 89], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 220, 167, 236], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 167, 12, 240], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1043659104, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 167, 132, 139, 96, 249, 52, 62], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 29, 177, 167, 198], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 449014260, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 109, 203, 167, 188, 206, 244, 105, 195, 26], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 210, 167, 20, 151], OperandSize::Qword)
}


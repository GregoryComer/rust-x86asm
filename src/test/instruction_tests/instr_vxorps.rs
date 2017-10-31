use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 87, 211], OperandSize::Dword)
}

#[test]
fn vxorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1557646580, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 87, 12, 221, 244, 204, 215, 92], OperandSize::Dword)
}

#[test]
fn vxorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 87, 236], OperandSize::Qword)
}

#[test]
fn vxorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1976886272, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 87, 172, 217, 0, 228, 212, 117], OperandSize::Qword)
}

#[test]
fn vxorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 237], OperandSize::Dword)
}

#[test]
fn vxorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 22477812, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 87, 190, 244, 251, 86, 1], OperandSize::Dword)
}

#[test]
fn vxorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 87, 242], OperandSize::Qword)
}

#[test]
fn vxorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 579678347, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 87, 140, 194, 139, 48, 141, 34], OperandSize::Qword)
}

#[test]
fn vxorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 139, 87, 219], OperandSize::Dword)
}

#[test]
fn vxorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 141, 87, 28, 115], OperandSize::Dword)
}

#[test]
fn vxorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 154, 87, 25], OperandSize::Dword)
}

#[test]
fn vxorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 124, 130, 87, 229], OperandSize::Qword)
}

#[test]
fn vxorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1308727321, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 100, 131, 87, 148, 155, 25, 152, 1, 78], OperandSize::Qword)
}

#[test]
fn vxorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1547564003, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 158, 87, 28, 125, 227, 243, 61, 92], OperandSize::Qword)
}

#[test]
fn vxorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 175, 87, 241], OperandSize::Dword)
}

#[test]
fn vxorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 519169125, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 87, 60, 205, 101, 228, 241, 30], OperandSize::Dword)
}

#[test]
fn vxorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 83222436, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 189, 87, 187, 164, 223, 245, 4], OperandSize::Dword)
}

#[test]
fn vxorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 28, 164, 87, 194], OperandSize::Qword)
}

#[test]
fn vxorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 2131561375, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 84, 171, 87, 188, 183, 159, 11, 13, 127], OperandSize::Qword)
}

#[test]
fn vxorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 1553711532, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 12, 191, 87, 140, 95, 172, 193, 155, 92], OperandSize::Qword)
}

#[test]
fn vxorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 201, 87, 208], OperandSize::Dword)
}

#[test]
fn vxorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 716694231, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 76, 201, 87, 12, 253, 215, 226, 183, 42], OperandSize::Dword)
}

#[test]
fn vxorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 629017844, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 219, 87, 162, 244, 12, 126, 37], OperandSize::Dword)
}

#[test]
fn vxorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 116, 193, 87, 216], OperandSize::Qword)
}

#[test]
fn vxorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RAX, 1924346826, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 207, 87, 168, 202, 51, 179, 114], OperandSize::Qword)
}

#[test]
fn vxorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 519645318, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 20, 212, 87, 52, 141, 134, 40, 249, 30], OperandSize::Qword)
}


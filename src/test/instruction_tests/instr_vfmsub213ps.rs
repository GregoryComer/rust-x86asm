use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 170, 241], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 170, 51], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 170, 196], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 1682764451, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 170, 150, 163, 242, 76, 100], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 170, 218], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 358325070, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 170, 166, 78, 155, 91, 21], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 170, 226], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 325952171, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 170, 140, 198, 171, 162, 109, 19], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 170, 196], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 170, 36, 73], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 1467229215, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 157, 170, 159, 31, 36, 116, 87], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 29, 134, 170, 235], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 105946266, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 139, 170, 28, 93, 154, 156, 80, 6], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 345422860, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 146, 170, 28, 93, 12, 188, 150, 20], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 170, 241], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 170, 52, 130], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 1370390729, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 187, 170, 180, 131, 201, 128, 174, 81], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 5, 163, 170, 206], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RDI, 1689818612, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 167, 170, 143, 244, 149, 184, 100], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 53, 189, 170, 52, 70], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 252, 170, 210], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 399675170, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 170, 180, 126, 34, 143, 210, 23], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1683995994, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 219, 170, 12, 125, 90, 189, 95, 100], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 77, 146, 170, 241], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1936893900, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 53, 193, 170, 164, 153, 204, 167, 114, 115], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDX, 1249445587, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 5, 209, 170, 162, 211, 6, 121, 74], OperandSize::Qword)
}


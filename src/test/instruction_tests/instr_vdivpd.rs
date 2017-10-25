use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 94, 207], OperandSize::Dword)
}

#[test]
fn vdivpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1941975686, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 94, 138, 134, 50, 192, 115], OperandSize::Dword)
}

#[test]
fn vdivpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 94, 228], OperandSize::Qword)
}

#[test]
fn vdivpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1927339803, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 94, 4, 157, 27, 223, 224, 114], OperandSize::Qword)
}

#[test]
fn vdivpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 94, 224], OperandSize::Dword)
}

#[test]
fn vdivpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 94, 35], OperandSize::Dword)
}

#[test]
fn vdivpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 94, 227], OperandSize::Qword)
}

#[test]
fn vdivpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 1531119127, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 162, 23, 6, 67, 91], OperandSize::Qword)
}

#[test]
fn vdivpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 139, 94, 193], OperandSize::Dword)
}

#[test]
fn vdivpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 2285015, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 94, 128, 215, 221, 34, 0], OperandSize::Dword)
}

#[test]
fn vdivpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1275137040, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 154, 94, 168, 16, 12, 1, 76], OperandSize::Dword)
}

#[test]
fn vdivpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 141, 143, 94, 245], OperandSize::Qword)
}

#[test]
fn vdivpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 189, 143, 94, 36, 211], OperandSize::Qword)
}

#[test]
fn vdivpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 21177415, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 245, 159, 94, 28, 221, 71, 36, 67, 1], OperandSize::Qword)
}

#[test]
fn vdivpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 94, 245], OperandSize::Dword)
}

#[test]
fn vdivpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 94, 10], OperandSize::Dword)
}

#[test]
fn vdivpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 189, 94, 12, 73], OperandSize::Dword)
}

#[test]
fn vdivpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 181, 167, 94, 234], OperandSize::Qword)
}

#[test]
fn vdivpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 213, 173, 94, 44, 83], OperandSize::Qword)
}

#[test]
fn vdivpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 941670157, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 173, 181, 94, 156, 94, 13, 191, 32, 56], OperandSize::Qword)
}

#[test]
fn vdivpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 220, 94, 217], OperandSize::Dword)
}

#[test]
fn vdivpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1258295768, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 207, 94, 164, 90, 216, 17, 0, 75], OperandSize::Dword)
}

#[test]
fn vdivpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 223, 94, 40], OperandSize::Dword)
}

#[test]
fn vdivpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 157, 223, 94, 216], OperandSize::Qword)
}

#[test]
fn vdivpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 141, 202, 94, 6], OperandSize::Qword)
}

#[test]
fn vdivpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RBX, 15762529, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 245, 211, 94, 147, 97, 132, 240, 0], OperandSize::Qword)
}


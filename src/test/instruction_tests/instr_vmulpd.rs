use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 89, 253], OperandSize::Dword)
}

#[test]
fn vmulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 89, 4, 191], OperandSize::Dword)
}

#[test]
fn vmulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 89, 241], OperandSize::Qword)
}

#[test]
fn vmulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 89, 44, 210], OperandSize::Qword)
}

#[test]
fn vmulpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 89, 207], OperandSize::Dword)
}

#[test]
fn vmulpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 790659317, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 175, 245, 128, 32, 47], OperandSize::Dword)
}

#[test]
fn vmulpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 202], OperandSize::Qword)
}

#[test]
fn vmulpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RAX, 699386690, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 128, 66, 203, 175, 41], OperandSize::Qword)
}

#[test]
fn vmulpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 89, 196], OperandSize::Dword)
}

#[test]
fn vmulpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 711669354, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 141, 89, 36, 125, 106, 54, 107, 42], OperandSize::Dword)
}

#[test]
fn vmulpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 155, 89, 3], OperandSize::Dword)
}

#[test]
fn vmulpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 157, 129, 89, 202], OperandSize::Qword)
}

#[test]
fn vmulpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1279050174, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 133, 134, 89, 132, 203, 190, 193, 60, 76], OperandSize::Qword)
}

#[test]
fn vmulpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 157, 156, 89, 54], OperandSize::Qword)
}

#[test]
fn vmulpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 89, 236], OperandSize::Dword)
}

#[test]
fn vmulpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 89, 4, 113], OperandSize::Dword)
}

#[test]
fn vmulpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 186, 89, 20, 152], OperandSize::Dword)
}

#[test]
fn vmulpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 173, 164, 89, 217], OperandSize::Qword)
}

#[test]
fn vmulpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RBX, 1257686658, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 166, 89, 155, 130, 198, 246, 74], OperandSize::Qword)
}

#[test]
fn vmulpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 920818425, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 221, 181, 89, 4, 149, 249, 146, 226, 54], OperandSize::Qword)
}

#[test]
fn vmulpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 190, 89, 195], OperandSize::Dword)
}

#[test]
fn vmulpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 531388948, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 205, 89, 140, 223, 20, 90, 172, 31], OperandSize::Dword)
}

#[test]
fn vmulpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 1423278032, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 222, 89, 163, 208, 127, 213, 84], OperandSize::Dword)
}

#[test]
fn vmulpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 149, 253, 89, 218], OperandSize::Qword)
}

#[test]
fn vmulpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1053027354, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 173, 197, 89, 140, 75, 26, 236, 195, 62], OperandSize::Qword)
}

#[test]
fn vmulpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1160408141, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 141, 210, 89, 156, 115, 77, 108, 42, 69], OperandSize::Qword)
}


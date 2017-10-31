use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 69, 226], OperandSize::Dword)
}

#[test]
fn vpsrlvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1963927527, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 69, 172, 183, 231, 39, 15, 117], OperandSize::Dword)
}

#[test]
fn vpsrlvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 69, 228], OperandSize::Qword)
}

#[test]
fn vpsrlvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 69, 28, 251], OperandSize::Qword)
}

#[test]
fn vpsrlvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 69, 229], OperandSize::Dword)
}

#[test]
fn vpsrlvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 399617645, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 69, 148, 64, 109, 174, 209, 23], OperandSize::Dword)
}

#[test]
fn vpsrlvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 69, 236], OperandSize::Qword)
}

#[test]
fn vpsrlvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 69, 28, 210], OperandSize::Qword)
}

#[test]
fn vpsrlvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 69, 245], OperandSize::Dword)
}

#[test]
fn vpsrlvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1853096128, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 69, 20, 157, 192, 0, 116, 110], OperandSize::Dword)
}

#[test]
fn vpsrlvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 69, 4, 115], OperandSize::Dword)
}

#[test]
fn vpsrlvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 85, 132, 69, 249], OperandSize::Qword)
}

#[test]
fn vpsrlvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 129, 69, 40], OperandSize::Qword)
}

#[test]
fn vpsrlvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1392032515, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 109, 151, 69, 20, 125, 3, 187, 248, 82], OperandSize::Qword)
}

#[test]
fn vpsrlvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 171, 69, 244], OperandSize::Dword)
}

#[test]
fn vpsrlvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 631741195, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 69, 186, 11, 155, 167, 37], OperandSize::Dword)
}

#[test]
fn vpsrlvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 69, 20, 79], OperandSize::Dword)
}

#[test]
fn vpsrlvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 85, 174, 69, 231], OperandSize::Qword)
}

#[test]
fn vpsrlvd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1646676397, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 53, 170, 69, 188, 199, 173, 73, 38, 98], OperandSize::Qword)
}

#[test]
fn vpsrlvd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 29, 191, 69, 8], OperandSize::Qword)
}

#[test]
fn vpsrlvd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 201, 69, 195], OperandSize::Dword)
}

#[test]
fn vpsrlvd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 126608113, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 69, 172, 130, 241, 226, 139, 7], OperandSize::Dword)
}

#[test]
fn vpsrlvd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 885922359, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 220, 69, 132, 246, 55, 26, 206, 52], OperandSize::Dword)
}

#[test]
fn vpsrlvd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 77, 207, 69, 207], OperandSize::Qword)
}

#[test]
fn vpsrlvd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RDI, 2133386922, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 45, 197, 69, 175, 170, 230, 40, 127], OperandSize::Qword)
}

#[test]
fn vpsrlvd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 53, 218, 69, 36, 122], OperandSize::Qword)
}


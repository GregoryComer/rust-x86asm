use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 251, 205], OperandSize::Dword)
}

#[test]
fn vpsubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 1134320562, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 251, 179, 178, 91, 156, 67], OperandSize::Dword)
}

#[test]
fn vpsubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 251, 208], OperandSize::Qword)
}

#[test]
fn vpsubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 251, 62], OperandSize::Qword)
}

#[test]
fn vpsubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 251, 241], OperandSize::Dword)
}

#[test]
fn vpsubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ESI, 2003203665, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 251, 166, 81, 118, 102, 119], OperandSize::Dword)
}

#[test]
fn vpsubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 251, 245], OperandSize::Qword)
}

#[test]
fn vpsubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 4, 88], OperandSize::Qword)
}

#[test]
fn vpsubq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 251, 200], OperandSize::Dword)
}

#[test]
fn vpsubq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 2011211431, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 251, 140, 182, 167, 166, 224, 119], OperandSize::Dword)
}

#[test]
fn vpsubq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 1707393368, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 153, 251, 147, 88, 193, 196, 101], OperandSize::Dword)
}

#[test]
fn vpsubq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 141, 129, 251, 254], OperandSize::Qword)
}

#[test]
fn vpsubq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 134074594, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 165, 132, 251, 132, 242, 226, 208, 253, 7], OperandSize::Qword)
}

#[test]
fn vpsubq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 181, 155, 251, 33], OperandSize::Qword)
}

#[test]
fn vpsubq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 251, 249], OperandSize::Dword)
}

#[test]
fn vpsubq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1486461415, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 251, 164, 202, 231, 153, 153, 88], OperandSize::Dword)
}

#[test]
fn vpsubq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 837209121, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 251, 4, 77, 33, 204, 230, 49], OperandSize::Dword)
}

#[test]
fn vpsubq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 197, 169, 251, 226], OperandSize::Qword)
}

#[test]
fn vpsubq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RDX, 1130031353, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 213, 165, 251, 154, 249, 232, 90, 67], OperandSize::Qword)
}

#[test]
fn vpsubq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 189, 251, 52, 145], OperandSize::Qword)
}

#[test]
fn vpsubq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 203, 251, 203], OperandSize::Dword)
}

#[test]
fn vpsubq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1313458737, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 251, 28, 157, 49, 202, 73, 78], OperandSize::Dword)
}

#[test]
fn vpsubq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 370434010, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 221, 251, 164, 242, 218, 95, 20, 22], OperandSize::Dword)
}

#[test]
fn vpsubq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 213, 204, 251, 239], OperandSize::Qword)
}

#[test]
fn vpsubq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 157, 203, 251, 44, 247], OperandSize::Qword)
}

#[test]
fn vpsubq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1869484393, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 221, 251, 140, 151, 105, 17, 110, 111], OperandSize::Qword)
}


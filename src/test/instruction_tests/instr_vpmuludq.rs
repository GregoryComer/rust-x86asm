use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 244, 241], OperandSize::Dword)
}

#[test]
fn vpmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 244, 48], OperandSize::Dword)
}

#[test]
fn vpmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 244, 222], OperandSize::Qword)
}

#[test]
fn vpmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDI, 990560174, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 244, 143, 174, 191, 10, 59], OperandSize::Qword)
}

#[test]
fn vpmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 244, 201], OperandSize::Dword)
}

#[test]
fn vpmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 244, 56], OperandSize::Dword)
}

#[test]
fn vpmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 244, 242], OperandSize::Qword)
}

#[test]
fn vpmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1223037727, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 244, 12, 213, 31, 19, 230, 72], OperandSize::Qword)
}

#[test]
fn vpmuludq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 139, 244, 211], OperandSize::Dword)
}

#[test]
fn vpmuludq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 1594186411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 244, 158, 171, 90, 5, 95], OperandSize::Dword)
}

#[test]
fn vpmuludq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 159, 244, 12, 178], OperandSize::Dword)
}

#[test]
fn vpmuludq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 149, 139, 244, 209], OperandSize::Qword)
}

#[test]
fn vpmuludq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 221, 132, 244, 58], OperandSize::Qword)
}

#[test]
fn vpmuludq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RDI, 894269130, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 237, 148, 244, 175, 202, 118, 77, 53], OperandSize::Qword)
}

#[test]
fn vpmuludq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 175, 244, 192], OperandSize::Dword)
}

#[test]
fn vpmuludq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 172, 244, 18], OperandSize::Dword)
}

#[test]
fn vpmuludq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 72763399, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 189, 244, 140, 206, 7, 72, 86, 4], OperandSize::Dword)
}

#[test]
fn vpmuludq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 237, 164, 244, 224], OperandSize::Qword)
}

#[test]
fn vpmuludq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 221, 165, 244, 44, 247], OperandSize::Qword)
}

#[test]
fn vpmuludq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 689813226, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 141, 183, 244, 188, 222, 234, 182, 29, 41], OperandSize::Qword)
}

#[test]
fn vpmuludq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 205, 244, 234], OperandSize::Dword)
}

#[test]
fn vpmuludq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 207, 244, 12, 65], OperandSize::Dword)
}

#[test]
fn vpmuludq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 237766529, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 223, 244, 172, 78, 129, 7, 44, 14], OperandSize::Dword)
}

#[test]
fn vpmuludq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 244, 195], OperandSize::Qword)
}

#[test]
fn vpmuludq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 237, 206, 244, 18], OperandSize::Qword)
}

#[test]
fn vpmuludq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 918024638, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 237, 215, 244, 180, 195, 190, 241, 183, 54], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 239, 213], OperandSize::Dword)
}

#[test]
fn vpxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1925345804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 239, 12, 213, 12, 114, 194, 114], OperandSize::Dword)
}

#[test]
fn vpxorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 155, 239, 4, 121], OperandSize::Dword)
}

#[test]
fn vpxorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 165, 132, 239, 245], OperandSize::Qword)
}

#[test]
fn vpxorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 196607468, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 213, 130, 239, 188, 95, 236, 253, 183, 11], OperandSize::Qword)
}

#[test]
fn vpxorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDX, 999658108, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 197, 149, 239, 146, 124, 146, 149, 59], OperandSize::Qword)
}

#[test]
fn vpxorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 239, 219], OperandSize::Dword)
}

#[test]
fn vpxorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 169, 239, 4, 187], OperandSize::Dword)
}

#[test]
fn vpxorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 828548130, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 189, 239, 60, 245, 34, 164, 98, 49], OperandSize::Dword)
}

#[test]
fn vpxorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 165, 172, 239, 224], OperandSize::Qword)
}

#[test]
fn vpxorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 300869757, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 133, 173, 239, 20, 221, 125, 232, 238, 17], OperandSize::Qword)
}

#[test]
fn vpxorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 410649984, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 197, 191, 239, 60, 221, 128, 5, 122, 24], OperandSize::Qword)
}

#[test]
fn vpxorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 239, 247], OperandSize::Dword)
}

#[test]
fn vpxorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 402787557, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 239, 132, 210, 229, 12, 2, 24], OperandSize::Dword)
}

#[test]
fn vpxorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 222, 239, 20, 64], OperandSize::Dword)
}

#[test]
fn vpxorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 141, 193, 239, 208], OperandSize::Qword)
}

#[test]
fn vpxorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 253, 194, 239, 27], OperandSize::Qword)
}

#[test]
fn vpxorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 742439354, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 157, 220, 239, 60, 85, 186, 185, 64, 44], OperandSize::Qword)
}


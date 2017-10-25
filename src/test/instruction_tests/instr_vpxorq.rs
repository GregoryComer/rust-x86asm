use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 138, 239, 244], OperandSize::Dword)
}

#[test]
fn vpxorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 239, 36, 126], OperandSize::Dword)
}

#[test]
fn vpxorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1007890903, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 158, 239, 52, 189, 215, 49, 19, 60], OperandSize::Dword)
}

#[test]
fn vpxorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 173, 133, 239, 199], OperandSize::Qword)
}

#[test]
fn vpxorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RAX, 2055364379, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 134, 239, 176, 27, 95, 130, 122], OperandSize::Qword)
}

#[test]
fn vpxorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 173, 155, 239, 47], OperandSize::Qword)
}

#[test]
fn vpxorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 170, 239, 204], OperandSize::Dword)
}

#[test]
fn vpxorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 361456199, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 239, 60, 245, 71, 98, 139, 21], OperandSize::Dword)
}

#[test]
fn vpxorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 187, 239, 36, 183], OperandSize::Dword)
}

#[test]
fn vpxorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 173, 164, 239, 244], OperandSize::Qword)
}

#[test]
fn vpxorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDI, 978419406, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 205, 170, 239, 175, 206, 126, 81, 58], OperandSize::Qword)
}

#[test]
fn vpxorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 822090900, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 213, 189, 239, 163, 148, 28, 0, 49], OperandSize::Qword)
}

#[test]
fn vpxorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 205, 239, 234], OperandSize::Dword)
}

#[test]
fn vpxorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 239, 60, 136], OperandSize::Dword)
}

#[test]
fn vpxorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1367429779, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 219, 239, 164, 78, 147, 82, 129, 81], OperandSize::Dword)
}

#[test]
fn vpxorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 181, 202, 239, 241], OperandSize::Qword)
}

#[test]
fn vpxorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 237, 193, 239, 50], OperandSize::Qword)
}

#[test]
fn vpxorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 381531810, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 181, 214, 239, 172, 209, 162, 182, 189, 22], OperandSize::Qword)
}


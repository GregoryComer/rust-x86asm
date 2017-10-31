use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 57, 249], OperandSize::Dword)
}

#[test]
fn vpminsq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 57, 20, 147], OperandSize::Dword)
}

#[test]
fn vpminsq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 224663787, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 159, 57, 60, 77, 235, 24, 100, 13], OperandSize::Dword)
}

#[test]
fn vpminsq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 245, 133, 57, 232], OperandSize::Qword)
}

#[test]
fn vpminsq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1074220390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 173, 135, 57, 156, 127, 102, 77, 7, 64], OperandSize::Qword)
}

#[test]
fn vpminsq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 402381001, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 145, 57, 36, 69, 201, 216, 251, 23], OperandSize::Qword)
}

#[test]
fn vpminsq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 57, 233], OperandSize::Dword)
}

#[test]
fn vpminsq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1905174863, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 57, 44, 157, 79, 169, 142, 113], OperandSize::Dword)
}

#[test]
fn vpminsq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 57, 34], OperandSize::Dword)
}

#[test]
fn vpminsq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 221, 169, 57, 199], OperandSize::Qword)
}

#[test]
fn vpminsq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 791795802, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 237, 161, 57, 132, 152, 90, 216, 49, 47], OperandSize::Qword)
}

#[test]
fn vpminsq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 157, 191, 57, 23], OperandSize::Qword)
}

#[test]
fn vpminsq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 57, 200], OperandSize::Dword)
}

#[test]
fn vpminsq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 57, 4, 146], OperandSize::Dword)
}

#[test]
fn vpminsq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 222, 57, 28, 216], OperandSize::Dword)
}

#[test]
fn vpminsq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 229, 201, 57, 255], OperandSize::Qword)
}

#[test]
fn vpminsq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 788776368, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 213, 204, 57, 20, 133, 176, 197, 3, 47], OperandSize::Qword)
}

#[test]
fn vpminsq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RDX, 2127792787, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 57, 138, 147, 138, 211, 126], OperandSize::Qword)
}


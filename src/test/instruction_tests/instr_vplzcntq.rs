use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 68, 249], OperandSize::Dword)
}

#[test]
fn vplzcntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1984054328, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 68, 164, 81, 56, 68, 66, 118], OperandSize::Dword)
}

#[test]
fn vplzcntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1321873982, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 68, 156, 120, 62, 50, 202, 78], OperandSize::Dword)
}

#[test]
fn vplzcntq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 253, 137, 68, 211], OperandSize::Qword)
}

#[test]
fn vplzcntq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM12)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 142, 68, 34], OperandSize::Qword)
}

#[test]
fn vplzcntq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1566181747, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 154, 68, 52, 149, 115, 9, 90, 93], OperandSize::Qword)
}

#[test]
fn vplzcntq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 68, 214], OperandSize::Dword)
}

#[test]
fn vplzcntq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 68, 7], OperandSize::Dword)
}

#[test]
fn vplzcntq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 68, 44, 223], OperandSize::Dword)
}

#[test]
fn vplzcntq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 253, 173, 68, 246], OperandSize::Qword)
}

#[test]
fn vplzcntq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectDisplaced(RAX, 880800107, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 171, 68, 168, 107, 241, 127, 52], OperandSize::Qword)
}

#[test]
fn vplzcntq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1264823911, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 187, 68, 60, 125, 103, 174, 99, 75], OperandSize::Qword)
}

#[test]
fn vplzcntq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 68, 237], OperandSize::Dword)
}

#[test]
fn vplzcntq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 68, 44, 178], OperandSize::Dword)
}

#[test]
fn vplzcntq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 68, 60, 81], OperandSize::Dword)
}

#[test]
fn vplzcntq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 253, 204, 68, 222], OperandSize::Qword)
}

#[test]
fn vplzcntq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 253, 207, 68, 60, 131], OperandSize::Qword)
}

#[test]
fn vplzcntq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 443136140, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 253, 221, 68, 140, 185, 140, 184, 105, 26], OperandSize::Qword)
}


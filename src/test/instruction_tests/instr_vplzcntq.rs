use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 68, 251], OperandSize::Dword)
}

#[test]
fn vplzcntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 68, 28, 82], OperandSize::Dword)
}

#[test]
fn vplzcntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 374786360, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 68, 148, 89, 56, 201, 86, 22], OperandSize::Dword)
}

#[test]
fn vplzcntq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 253, 140, 68, 231], OperandSize::Qword)
}

#[test]
fn vplzcntq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 798319582, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 140, 68, 132, 193, 222, 99, 149, 47], OperandSize::Qword)
}

#[test]
fn vplzcntq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1713223803, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 159, 68, 156, 176, 123, 184, 29, 102], OperandSize::Qword)
}

#[test]
fn vplzcntq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 68, 223], OperandSize::Dword)
}

#[test]
fn vplzcntq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 68, 36, 78], OperandSize::Dword)
}

#[test]
fn vplzcntq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 68, 2], OperandSize::Dword)
}

#[test]
fn vplzcntq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 253, 173, 68, 211], OperandSize::Qword)
}

#[test]
fn vplzcntq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 634037826, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 68, 148, 142, 66, 166, 202, 37], OperandSize::Qword)
}

#[test]
fn vplzcntq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM29)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 189, 68, 44, 177], OperandSize::Qword)
}

#[test]
fn vplzcntq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 68, 228], OperandSize::Dword)
}

#[test]
fn vplzcntq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 68, 60, 123], OperandSize::Dword)
}

#[test]
fn vplzcntq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDX, 1071512641, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 68, 178, 65, 252, 221, 63], OperandSize::Dword)
}

#[test]
fn vplzcntq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 253, 206, 68, 248], OperandSize::Qword)
}

#[test]
fn vplzcntq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RDI, 300353627, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 204, 68, 191, 91, 8, 231, 17], OperandSize::Qword)
}

#[test]
fn vplzcntq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM19)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 221, 68, 25], OperandSize::Qword)
}


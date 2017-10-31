use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 68, 246], OperandSize::Dword)
}

#[test]
fn vplzcntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 688756183, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 68, 60, 189, 215, 149, 13, 41], OperandSize::Dword)
}

#[test]
fn vplzcntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1043041112, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 157, 68, 52, 117, 88, 139, 43, 62], OperandSize::Dword)
}

#[test]
fn vplzcntq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 68, 243], OperandSize::Qword)
}

#[test]
fn vplzcntq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1407563091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 68, 60, 77, 83, 181, 229, 83], OperandSize::Qword)
}

#[test]
fn vplzcntq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1563102056, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 158, 68, 140, 67, 104, 11, 43, 93], OperandSize::Qword)
}

#[test]
fn vplzcntq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 68, 202], OperandSize::Dword)
}

#[test]
fn vplzcntq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 24769458, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 68, 140, 89, 178, 243, 121, 1], OperandSize::Dword)
}

#[test]
fn vplzcntq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 68, 19], OperandSize::Dword)
}

#[test]
fn vplzcntq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 253, 172, 68, 228], OperandSize::Qword)
}

#[test]
fn vplzcntq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 253, 169, 68, 20, 200], OperandSize::Qword)
}

#[test]
fn vplzcntq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 186, 68, 1], OperandSize::Qword)
}

#[test]
fn vplzcntq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 68, 223], OperandSize::Dword)
}

#[test]
fn vplzcntq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 68, 51], OperandSize::Dword)
}

#[test]
fn vplzcntq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 668952129, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 68, 44, 117, 65, 102, 223, 39], OperandSize::Dword)
}

#[test]
fn vplzcntq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 203, 68, 212], OperandSize::Qword)
}

#[test]
fn vplzcntq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 567049105, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 203, 68, 44, 253, 145, 123, 204, 33], OperandSize::Qword)
}

#[test]
fn vplzcntq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1806505115, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 222, 68, 164, 75, 155, 20, 173, 107], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 78, 234], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 550985368, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 78, 142, 152, 94, 215, 32], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 191768988, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 78, 180, 146, 156, 41, 110, 11], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 142, 78, 226], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 560022844, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 253, 140, 78, 188, 150, 60, 69, 97, 33], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 78, 44, 143], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 78, 226], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 78, 59], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EBX, 1522700600, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 78, 187, 56, 145, 194, 90], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 253, 169, 78, 229], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM22)), operand2: Some(IndirectDisplaced(RBX, 695344678, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 253, 173, 78, 179, 38, 30, 114, 41], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1475541694, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 187, 78, 188, 203, 190, 250, 242, 87], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 78, 243], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1299193816, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 78, 148, 243, 216, 31, 112, 77], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EAX, 399261846, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 78, 128, 150, 64, 204, 23], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 253, 205, 78, 203], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1942998788, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 206, 78, 4, 117, 4, 207, 207, 115], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 78, 60, 199], OperandSize::Qword)
}


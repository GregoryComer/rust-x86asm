use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 66, 251], OperandSize::Dword)
}

#[test]
fn vgetexppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1286312965, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 66, 20, 77, 5, 148, 171, 76], OperandSize::Dword)
}

#[test]
fn vgetexppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 157, 66, 62], OperandSize::Dword)
}

#[test]
fn vgetexppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 253, 142, 66, 252], OperandSize::Qword)
}

#[test]
fn vgetexppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RDX, 1473613135, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 141, 66, 170, 79, 141, 213, 87], OperandSize::Qword)
}

#[test]
fn vgetexppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectDisplaced(RDI, 1673350969, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 158, 66, 159, 57, 79, 189, 99], OperandSize::Qword)
}

#[test]
fn vgetexppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 66, 237], OperandSize::Dword)
}

#[test]
fn vgetexppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 66, 44, 217], OperandSize::Dword)
}

#[test]
fn vgetexppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 186, 66, 20, 87], OperandSize::Dword)
}

#[test]
fn vgetexppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 253, 175, 66, 213], OperandSize::Qword)
}

#[test]
fn vgetexppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM15)), operand2: Some(IndirectDisplaced(RSI, 2099890615, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 174, 66, 190, 183, 201, 41, 125], OperandSize::Qword)
}

#[test]
fn vgetexppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(YMM25)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 188, 66, 15], OperandSize::Qword)
}

#[test]
fn vgetexppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 66, 232], OperandSize::Dword)
}

#[test]
fn vgetexppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1290156946, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 66, 60, 157, 146, 59, 230, 76], OperandSize::Dword)
}

#[test]
fn vgetexppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1591364448, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 66, 140, 155, 96, 75, 218, 94], OperandSize::Dword)
}

#[test]
fn vgetexppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 253, 153, 66, 196], OperandSize::Qword)
}

#[test]
fn vgetexppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 780254131, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 66, 20, 149, 179, 187, 129, 46], OperandSize::Qword)
}

#[test]
fn vgetexppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPD, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 220, 66, 24], OperandSize::Qword)
}


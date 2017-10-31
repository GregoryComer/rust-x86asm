use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 115, 211, 88], OperandSize::Dword)
}

#[test]
fn vpsrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 213, 61], OperandSize::Qword)
}

#[test]
fn vpsrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 210, 57], OperandSize::Dword)
}

#[test]
fn vpsrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 115, 208, 14], OperandSize::Qword)
}

#[test]
fn vpsrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 211, 197], OperandSize::Dword)
}

#[test]
fn vpsrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1215180642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 211, 20, 157, 98, 47, 110, 72], OperandSize::Dword)
}

#[test]
fn vpsrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 211, 218], OperandSize::Qword)
}

#[test]
fn vpsrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 211, 52, 223], OperandSize::Qword)
}

#[test]
fn vpsrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 211, 235], OperandSize::Dword)
}

#[test]
fn vpsrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 211, 52, 198], OperandSize::Dword)
}

#[test]
fn vpsrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 211, 213], OperandSize::Qword)
}

#[test]
fn vpsrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 211, 1], OperandSize::Qword)
}

#[test]
fn vpsrlq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 211, 250], OperandSize::Dword)
}

#[test]
fn vpsrlq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 503708275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 211, 147, 115, 250, 5, 30], OperandSize::Dword)
}

#[test]
fn vpsrlq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 165, 131, 211, 247], OperandSize::Qword)
}

#[test]
fn vpsrlq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 205, 137, 211, 1], OperandSize::Qword)
}

#[test]
fn vpsrlq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 211, 247], OperandSize::Dword)
}

#[test]
fn vpsrlq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 651434052, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 172, 211, 128, 68, 24, 212, 38], OperandSize::Dword)
}

#[test]
fn vpsrlq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 245, 165, 211, 234], OperandSize::Qword)
}

#[test]
fn vpsrlq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 934986382, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 149, 161, 211, 156, 120, 142, 194, 186, 55], OperandSize::Qword)
}

#[test]
fn vpsrlq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 211, 198], OperandSize::Dword)
}

#[test]
fn vpsrlq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 1798270057, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 206, 211, 160, 105, 108, 47, 107], OperandSize::Dword)
}

#[test]
fn vpsrlq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 173, 204, 211, 238], OperandSize::Qword)
}

#[test]
fn vpsrlq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RDI, 993573450, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 157, 195, 211, 159, 74, 186, 56, 59], OperandSize::Qword)
}


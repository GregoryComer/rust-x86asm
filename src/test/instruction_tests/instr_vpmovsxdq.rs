use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 255], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1806599074, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 188, 127, 162, 131, 174, 107], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 216], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 62], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 240], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 52, 184], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 217], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1022606036, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 156, 142, 212, 186, 243, 60], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 37, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 670909734, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 37, 186, 38, 69, 253, 39], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 37, 207], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 37, 34], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 37, 197], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 2022966297, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 37, 156, 135, 25, 4, 148, 120], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 170, 37, 227], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 133667197, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 172, 37, 44, 69, 125, 153, 247, 7], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 37, 218], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 435101698, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 37, 132, 206, 2, 32, 239, 25], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 125, 205, 37, 216], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(RCX, 1718669724, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 37, 169, 156, 209, 112, 102], OperandSize::Qword)
}


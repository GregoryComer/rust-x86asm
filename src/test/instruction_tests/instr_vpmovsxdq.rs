use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsxdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 229], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 600518758, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 185, 102, 48, 203, 35], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 192], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 37, 20, 214], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 247], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1358115079, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 4, 125, 7, 49, 243, 80], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 215], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 37, 60, 66], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 37, 255], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1926966679, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 37, 60, 133, 151, 45, 219, 114], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 37, 249], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 412574149, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 142, 37, 164, 202, 197, 97, 151, 24], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 37, 217], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1194252490, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 37, 164, 83, 202, 216, 46, 71], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 125, 169, 37, 192], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 173, 37, 60, 113], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 37, 219], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 400298914, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 37, 188, 158, 162, 19, 220, 23], OperandSize::Dword)
}

#[test]
fn vpmovsxdq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 201, 37, 192], OperandSize::Qword)
}

#[test]
fn vpmovsxdq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSXDQ, operand1: Some(Direct(ZMM10)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 37, 16], OperandSize::Qword)
}


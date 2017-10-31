use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 61, 238], OperandSize::Dword)
}

#[test]
fn vpmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 61, 20, 82], OperandSize::Dword)
}

#[test]
fn vpmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 61, 218], OperandSize::Qword)
}

#[test]
fn vpmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 61, 12, 249], OperandSize::Qword)
}

#[test]
fn vpmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 61, 218], OperandSize::Dword)
}

#[test]
fn vpmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 61, 7], OperandSize::Dword)
}

#[test]
fn vpmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 61, 248], OperandSize::Qword)
}

#[test]
fn vpmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 61, 12, 64], OperandSize::Qword)
}

#[test]
fn vpmaxsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 61, 216], OperandSize::Dword)
}

#[test]
fn vpmaxsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1816149527, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 61, 180, 139, 23, 62, 64, 108], OperandSize::Dword)
}

#[test]
fn vpmaxsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 577707410, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 61, 148, 240, 146, 29, 111, 34], OperandSize::Dword)
}

#[test]
fn vpmaxsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 61, 134, 61, 207], OperandSize::Qword)
}

#[test]
fn vpmaxsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDX, 195268919, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 53, 134, 61, 162, 55, 145, 163, 11], OperandSize::Qword)
}

#[test]
fn vpmaxsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 1505975565, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 101, 155, 61, 166, 13, 93, 195, 89], OperandSize::Qword)
}


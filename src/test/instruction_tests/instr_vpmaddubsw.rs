use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 4, 206], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 769291606, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 4, 146, 86, 117, 218, 45], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 4, 202], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 856278760, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 4, 188, 143, 232, 198, 9, 51], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 4, 246], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 986597384, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 4, 188, 137, 8, 72, 206, 58], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 4, 239], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1416562143, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 4, 60, 181, 223, 5, 111, 84], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 4, 234], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 4, 52, 153], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 53, 135, 4, 239], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1009239581, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 21, 141, 4, 148, 121, 29, 198, 39, 60], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 4, 220], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 4, 49], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 101, 169, 4, 192], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 170059108, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 117, 174, 4, 156, 83, 100, 229, 34, 10], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 4, 245], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 207, 4, 58], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 101, 205, 4, 246], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 37, 203, 4, 12, 190], OperandSize::Qword)
}


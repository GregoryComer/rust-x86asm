use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 4, 243], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 4, 40], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 4, 224], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 4, 60, 249], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 4, 227], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 276846661, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 4, 162, 69, 88, 128, 16], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 4, 249], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 784622173, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 4, 140, 194, 93, 98, 196, 46], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 4, 207], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 972449600, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 4, 156, 184, 64, 103, 246, 57], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 85, 129, 4, 240], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 946917781, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 85, 139, 4, 164, 70, 149, 209, 112, 56], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 4, 252], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 4, 1], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 117, 165, 4, 202], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RCX, 837042488, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 37, 175, 4, 177, 56, 65, 228, 49], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 4, 218], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 4, 36, 145], OperandSize::Dword)
}

#[test]
fn vpmaddubsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 93, 201, 4, 238], OperandSize::Qword)
}

#[test]
fn vpmaddubsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 93, 204, 4, 54], OperandSize::Qword)
}


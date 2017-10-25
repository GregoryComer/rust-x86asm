use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 198], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 838278152, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 158, 8, 28, 247, 49], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 208], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 44, 134], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 250], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 758969151, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 163, 63, 243, 60, 45], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 245], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RAX, 1103303323, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 128, 155, 18, 195, 65], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 49, 214], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 49, 60, 88], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 125, 140, 49, 204], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 974635234, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 137, 49, 20, 149, 226, 192, 23, 58], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 49, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 49, 26], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 125, 171, 49, 199], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM20)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 174, 49, 39], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 49, 227], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2128048155, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 49, 52, 181, 27, 112, 215, 126], OperandSize::Dword)
}

#[test]
fn vpmovzxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 201, 49, 216], OperandSize::Qword)
}

#[test]
fn vpmovzxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1112789285, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 49, 132, 94, 37, 209, 83, 66], OperandSize::Qword)
}


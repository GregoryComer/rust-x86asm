use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 249], OperandSize::Dword)
}

fn vpmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 355623044, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 185, 132, 96, 50, 21], OperandSize::Dword)
}

fn vpmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 210], OperandSize::Qword)
}

fn vpmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 49, 56], OperandSize::Qword)
}

fn vpmovzxbd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 255], OperandSize::Dword)
}

fn vpmovzxbd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(ECX, 1929085838, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 177, 142, 131, 251, 114], OperandSize::Dword)
}

fn vpmovzxbd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 193], OperandSize::Qword)
}

fn vpmovzxbd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1418931782, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 49, 132, 90, 70, 46, 147, 84], OperandSize::Qword)
}

fn vpmovzxbd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 49, 212], OperandSize::Dword)
}

fn vpmovzxbd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1334001, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 49, 132, 183, 241, 90, 20, 0], OperandSize::Dword)
}

fn vpmovzxbd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 49, 216], OperandSize::Qword)
}

fn vpmovzxbd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1700270491, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 141, 49, 4, 253, 155, 17, 88, 101], OperandSize::Qword)
}

fn vpmovzxbd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 49, 223], OperandSize::Dword)
}

fn vpmovzxbd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 827558882, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 49, 140, 131, 226, 139, 83, 49], OperandSize::Dword)
}

fn vpmovzxbd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 125, 169, 49, 220], OperandSize::Qword)
}

fn vpmovzxbd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(YMM10)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 49, 22], OperandSize::Qword)
}

fn vpmovzxbd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 49, 221], OperandSize::Dword)
}

fn vpmovzxbd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 49, 40], OperandSize::Dword)
}

fn vpmovzxbd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 125, 205, 49, 226], OperandSize::Qword)
}

fn vpmovzxbd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 616061681, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 49, 164, 185, 241, 90, 184, 36], OperandSize::Qword)
}


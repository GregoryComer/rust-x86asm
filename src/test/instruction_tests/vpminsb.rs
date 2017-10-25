use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 56, 194], OperandSize::Dword)
}

fn vpminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 2094815365, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 56, 140, 185, 133, 88, 220, 124], OperandSize::Dword)
}

fn vpminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 56, 224], OperandSize::Qword)
}

fn vpminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1609268574, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 56, 156, 186, 94, 125, 235, 95], OperandSize::Qword)
}

fn vpminsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 56, 246], OperandSize::Dword)
}

fn vpminsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 373079032, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 56, 20, 189, 248, 187, 60, 22], OperandSize::Dword)
}

fn vpminsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 56, 219], OperandSize::Qword)
}

fn vpminsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 27349857, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 56, 36, 133, 97, 83, 161, 1], OperandSize::Qword)
}

fn vpminsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 141, 56, 217], OperandSize::Dword)
}

fn vpminsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 56, 49], OperandSize::Dword)
}

fn vpminsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 53, 133, 56, 234], OperandSize::Qword)
}

fn vpminsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 568078922, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 125, 141, 56, 188, 89, 74, 50, 220, 33], OperandSize::Qword)
}

fn vpminsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 56, 205], OperandSize::Dword)
}

fn vpminsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 612445103, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 56, 156, 139, 175, 43, 129, 36], OperandSize::Dword)
}

fn vpminsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 21, 162, 56, 246], OperandSize::Qword)
}

fn vpminsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 21305745, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 117, 163, 56, 172, 72, 145, 25, 69, 1], OperandSize::Qword)
}

fn vpminsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 56, 212], OperandSize::Dword)
}

fn vpminsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1651341707, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 207, 56, 170, 139, 121, 109, 98], OperandSize::Dword)
}

fn vpminsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 29, 201, 56, 219], OperandSize::Qword)
}

fn vpminsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RCX, 190995273, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 117, 203, 56, 169, 73, 91, 98, 11], OperandSize::Qword)
}


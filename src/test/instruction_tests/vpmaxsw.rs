use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 238, 250], OperandSize::Dword)
}

fn vpmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 238, 3], OperandSize::Dword)
}

fn vpmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 238, 246], OperandSize::Qword)
}

fn vpmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 238, 52, 134], OperandSize::Qword)
}

fn vpmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 238, 235], OperandSize::Dword)
}

fn vpmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ECX, 514018195, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 185, 147, 75, 163, 30], OperandSize::Dword)
}

fn vpmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 232], OperandSize::Qword)
}

fn vpmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1981144955, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 138, 123, 223, 21, 118], OperandSize::Qword)
}

fn vpmaxsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 238, 193], OperandSize::Dword)
}

fn vpmaxsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 238, 6], OperandSize::Dword)
}

fn vpmaxsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 101, 135, 238, 195], OperandSize::Qword)
}

fn vpmaxsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 45, 141, 238, 41], OperandSize::Qword)
}

fn vpmaxsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 238, 245], OperandSize::Dword)
}

fn vpmaxsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1174026601, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 238, 164, 214, 105, 57, 250, 69], OperandSize::Dword)
}

fn vpmaxsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 61, 164, 238, 212], OperandSize::Qword)
}

fn vpmaxsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RCX, 4442535, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 165, 238, 145, 167, 201, 67, 0], OperandSize::Qword)
}

fn vpmaxsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 203, 238, 218], OperandSize::Dword)
}

fn vpmaxsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 950154085, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 238, 52, 117, 101, 51, 162, 56], OperandSize::Dword)
}

fn vpmaxsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 37, 206, 238, 210], OperandSize::Qword)
}

fn vpmaxsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 550919743, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 29, 198, 238, 156, 73, 63, 94, 214, 32], OperandSize::Qword)
}


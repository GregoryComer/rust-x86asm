use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 62, 240], OperandSize::Dword)
}

fn vpmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1406505920, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 62, 52, 221, 192, 147, 213, 83], OperandSize::Dword)
}

fn vpmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 62, 203], OperandSize::Qword)
}

fn vpmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 1811330651, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 62, 139, 91, 182, 246, 107], OperandSize::Qword)
}

fn vpmaxuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 62, 202], OperandSize::Dword)
}

fn vpmaxuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 62, 36, 153], OperandSize::Dword)
}

fn vpmaxuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 62, 225], OperandSize::Qword)
}

fn vpmaxuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RAX, 307013520, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 62, 144, 144, 167, 76, 18], OperandSize::Qword)
}

fn vpmaxuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 62, 216], OperandSize::Dword)
}

fn vpmaxuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1018855427, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 142, 62, 140, 249, 3, 128, 186, 60], OperandSize::Dword)
}

fn vpmaxuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 29, 138, 62, 205], OperandSize::Qword)
}

fn vpmaxuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 77, 143, 62, 51], OperandSize::Qword)
}

fn vpmaxuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 62, 236], OperandSize::Dword)
}

fn vpmaxuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 62, 44, 139], OperandSize::Dword)
}

fn vpmaxuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 101, 169, 62, 237], OperandSize::Qword)
}

fn vpmaxuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 117, 162, 62, 20, 193], OperandSize::Qword)
}

fn vpmaxuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 62, 252], OperandSize::Dword)
}

fn vpmaxuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 62, 1], OperandSize::Dword)
}

fn vpmaxuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 61, 194, 62, 209], OperandSize::Qword)
}

fn vpmaxuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectDisplaced(RDX, 2096891965, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 13, 201, 62, 146, 61, 8, 252, 124], OperandSize::Qword)
}


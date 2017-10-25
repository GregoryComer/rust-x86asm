use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 61, 194], OperandSize::Dword)
}

fn vpmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 1400536867, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 61, 140, 210, 35, 127, 122, 83], OperandSize::Dword)
}

fn vpmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 61, 210], OperandSize::Qword)
}

fn vpmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 61, 55], OperandSize::Qword)
}

fn vpmaxsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 61, 246], OperandSize::Dword)
}

fn vpmaxsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 61, 52, 121], OperandSize::Dword)
}

fn vpmaxsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 61, 230], OperandSize::Qword)
}

fn vpmaxsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 2037617184, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 61, 140, 123, 32, 146, 115, 121], OperandSize::Qword)
}

fn vpmaxsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 61, 222], OperandSize::Dword)
}

fn vpmaxsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 61, 33], OperandSize::Dword)
}

fn vpmaxsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1825389390, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 155, 61, 151, 78, 59, 205, 108], OperandSize::Dword)
}

fn vpmaxsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 61, 137, 61, 214], OperandSize::Qword)
}

fn vpmaxsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RCX, 549476503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 93, 131, 61, 185, 151, 88, 192, 32], OperandSize::Qword)
}

fn vpmaxsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 774026336, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 21, 147, 61, 140, 80, 96, 180, 34, 46], OperandSize::Qword)
}


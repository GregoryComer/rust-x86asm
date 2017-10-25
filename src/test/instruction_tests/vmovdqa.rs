use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 202], OperandSize::Dword)
}

fn vmovdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 954090688, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 188, 78, 192, 68, 222, 56], OperandSize::Dword)
}

fn vmovdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 255], OperandSize::Qword)
}

fn vmovdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1314036091, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 44, 253, 123, 153, 82, 78], OperandSize::Qword)
}

fn vmovdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 206], OperandSize::Dword)
}

fn vmovdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 56], OperandSize::Dword)
}

fn vmovdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 200], OperandSize::Qword)
}

fn vmovdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 12, 194], OperandSize::Qword)
}

fn vmovdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 226], OperandSize::Dword)
}

fn vmovdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1505450080, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 172, 249, 96, 88, 187, 89], OperandSize::Dword)
}

fn vmovdqa_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 111, 230], OperandSize::Qword)
}

fn vmovdqa_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1679944330, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 127, 60, 213, 138, 234, 33, 100], OperandSize::Qword)
}

fn vmovdqa_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 242], OperandSize::Dword)
}

fn vmovdqa_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectDisplaced(ESI, 1179096733, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 142, 157, 150, 71, 70], OperandSize::Dword)
}

fn vmovdqa_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 111, 202], OperandSize::Qword)
}

fn vmovdqa_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA, operand1: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 127, 36, 194], OperandSize::Qword)
}


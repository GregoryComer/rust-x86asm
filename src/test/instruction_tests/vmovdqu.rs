use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 201], OperandSize::Dword)
}

fn vmovdqu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 2110879790, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 164, 129, 46, 120, 209, 125], OperandSize::Dword)
}

fn vmovdqu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 240], OperandSize::Qword)
}

fn vmovdqu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 2099058402, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 174, 226, 22, 29, 125], OperandSize::Qword)
}

fn vmovdqu_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 222], OperandSize::Dword)
}

fn vmovdqu_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 599492631, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 180, 217, 23, 136, 187, 35], OperandSize::Dword)
}

fn vmovdqu_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 250], OperandSize::Qword)
}

fn vmovdqu_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1816901355, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 20, 69, 235, 182, 75, 108], OperandSize::Qword)
}

fn vmovdqu_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 220], OperandSize::Dword)
}

fn vmovdqu_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectDisplaced(EAX, 1798039290, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 128, 250, 230, 43, 107], OperandSize::Dword)
}

fn vmovdqu_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 111, 218], OperandSize::Qword)
}

fn vmovdqu_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 127, 4, 201], OperandSize::Qword)
}

fn vmovdqu_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 208], OperandSize::Dword)
}

fn vmovdqu_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 52, 209], OperandSize::Dword)
}

fn vmovdqu_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 111, 239], OperandSize::Qword)
}

fn vmovdqu_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU, operand1: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 127, 47], OperandSize::Qword)
}


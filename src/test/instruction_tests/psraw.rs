use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM4)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 228, 127], OperandSize::Dword)
}

fn psraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM6)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 230, 0], OperandSize::Qword)
}

fn psraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 228, 5], OperandSize::Dword)
}

fn psraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 230, 97], OperandSize::Qword)
}

fn psraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 246], OperandSize::Dword)
}

fn psraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 679901829, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 28, 181, 133, 122, 134, 40], OperandSize::Dword)
}

fn psraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 212], OperandSize::Qword)
}

fn psraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 4, 248], OperandSize::Qword)
}

fn psraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 227], OperandSize::Dword)
}

fn psraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 812370076, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 175, 156, 200, 107, 48], OperandSize::Dword)
}

fn psraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 229], OperandSize::Qword)
}

fn psraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 52, 145], OperandSize::Qword)
}


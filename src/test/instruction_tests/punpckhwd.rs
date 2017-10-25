use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 245], OperandSize::Dword)
}

fn punpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 887010141, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 172, 150, 93, 179, 222, 52], OperandSize::Dword)
}

fn punpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 232], OperandSize::Qword)
}

fn punpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDI, 1078058679, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 151, 183, 222, 65, 64], OperandSize::Qword)
}

fn punpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 236], OperandSize::Dword)
}

fn punpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 47], OperandSize::Dword)
}

fn punpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 195], OperandSize::Qword)
}

fn punpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 12, 73], OperandSize::Qword)
}


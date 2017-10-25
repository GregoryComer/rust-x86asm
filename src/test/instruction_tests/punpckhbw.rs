use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 227], OperandSize::Dword)
}

fn punpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 11], OperandSize::Dword)
}

fn punpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 227], OperandSize::Qword)
}

fn punpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1007463874, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 28, 157, 194, 173, 12, 60], OperandSize::Qword)
}

fn punpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 227], OperandSize::Dword)
}

fn punpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EDI, 426277527, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 143, 151, 122, 104, 25], OperandSize::Dword)
}

fn punpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 238], OperandSize::Qword)
}

fn punpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 651064404, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 132, 95, 84, 116, 206, 38], OperandSize::Qword)
}


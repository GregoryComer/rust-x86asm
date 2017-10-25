use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pabsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 222], OperandSize::Dword)
}

fn pabsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 52, 150], OperandSize::Dword)
}

fn pabsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 217], OperandSize::Qword)
}

fn pabsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 30, 34], OperandSize::Qword)
}

fn pabsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 223], OperandSize::Dword)
}

fn pabsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 28, 194], OperandSize::Dword)
}

fn pabsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 207], OperandSize::Qword)
}

fn pabsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 2098159994, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 30, 142, 122, 97, 15, 125], OperandSize::Qword)
}


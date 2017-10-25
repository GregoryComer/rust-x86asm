use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmadd231ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 185, 225], OperandSize::Dword)
}

fn vfmadd231ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 185, 59], OperandSize::Dword)
}

fn vfmadd231ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 185, 254], OperandSize::Qword)
}

fn vfmadd231ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 1875055402, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 185, 159, 42, 19, 195, 111], OperandSize::Qword)
}

fn vfmadd231ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 186, 185, 201], OperandSize::Dword)
}

fn vfmadd231ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 777361948, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 185, 28, 205, 28, 154, 85, 46], OperandSize::Dword)
}

fn vfmadd231ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 13, 149, 185, 217], OperandSize::Qword)
}

fn vfmadd231ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231SS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 875710899, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 53, 129, 185, 12, 85, 179, 73, 50, 52], OperandSize::Qword)
}

